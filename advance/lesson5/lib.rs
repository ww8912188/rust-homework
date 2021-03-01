#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

    #[ink::contract]
    mod erc20 {
        use ink_storage::collections::HashMap as StorageHashMap;
        use ink_prelude::vec::Vec;
        //定义数据结构，应为要求实现的是ERC20标准，所以目前Substrate关于存储字符串浪费存储的设计哲学暂不考虑
        //OpenZeppelin https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol
        //EIP https://learnblockchain.cn/docs/eips/all.html#%E6%9C%80%E7%BB%88-final

        //类型拓展
        //Contracts may store types that are encodable and decodable with Parity Codec which includes the most common types
        // such as bool,u{8,16,32,64,128}, i{8,16,32,64,128}, String, tuples, and arrays.
        //ink! provides smart contracts Substrate specific types like AccountId, Balance, and Hash as if they were primitive types.
        // Also ink! provides storage types for more elaborate storage interactions through the storage module:
        // You can find all the supported Substrate types in crates/storage/src/lib.rs.

        #[ink(storage)]
        pub struct Erc20 {
            creator: AccountId,
            // 代币名称
            name: Vec<u8>,
            // 代币标识
            symbol: Vec<u8>,
            // 定义代币供应总量
            total_supply:Balance,
            //发行精度
            decimals:u8,
            // 存储各个账号的余额
            balances : StorageHashMap<AccountId, Balance>,
            // 授权某人可以使用自己的余额
            allowances : StorageHashMap<(AccountId, AccountId), Balance>
        }

    // 定义事件，ink(topic) 标识有需要通过这个字段查询时间的需求
    // 因为发行或者销毁的时候，from 或者 to 会是 none ，所以需要转账的 from 和 to 设置为 Option
    // 因为授权，两个账号必须存在，所以 owner 和 spender 不需要 Option
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    // 定义不同错误的的枚举类型，
    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
        OnlyForCreator,
    }

    // 定义返回类型，当有返回值也可能返回错误的函数，需要用 Result 类型返回
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// 初始化部署代币
        /// name : 代币名称，如 BitCoin
        /// symbol : 代币标识，如 BTC
        /// total_subbly : 总供应量
        /// decimals 如18位，最多不超过256
        #[ink(constructor)]
        pub fn new(name: Vec<u8>, symbol: Vec<u8>, total_supply: Balance,decimals:u8) -> Self {
            // 获取部署的调用者
            let caller = Self::env().caller();
            // 定义余额数据，将所有发行的代币，都放给部署账号
            let mut balances = StorageHashMap::new();
            balances.insert(caller, total_supply);
            // 定义数据存储
            let instance = Self {
                creator : caller, name, symbol, total_supply, balances,
                allowances: StorageHashMap::new(),
                decimals
            };
            // 触发转账事件，因为第一笔发行，也是一种转账
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            instance
        }

        /// 返回代币名称，如 BitCoin
        #[ink(message)]
        pub fn name(&self) -> Vec<u8>{
            self.name.clone()
        }

        /// 返回代币标识，比如 BTC
        #[ink(message)]
        pub fn symbol(&self) -> Vec<u8>{
            self.symbol.clone()
        }

        // 返回代币总供应量
        #[ink(message)]
        pub fn total_supply(&self) -> Balance{
            self.total_supply
        }

        // 返回代币精度
        #[ink(message)]
        pub fn decimals(&self) -> u8{
            self.decimals
        }


        /// 返回指定账号的余额
        #[ink(message)]
        pub fn balance_of(&self, of: AccountId) -> Balance{
            // 返回的值是 &Balance 的类型，所以需要 * 解引用
            // 可以使用 copied ，这样就不需要解引用了
            let balance = self.balances.get(&of).unwrap_or(&0);
            *balance
        }

        /// 向指定账号转账
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value:Balance) -> Result<()>{
            // 获取调用者
            let caller = Self::env().caller();
            self.transfer_from_to(Some(caller), Some(to), value)
        }

        /// 授权某账号可以使用自己的账户余额
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()>{
            let caller = Self::env().caller();
            // 插入授权的记录，授权是未来花费，所以不需要考虑当前是否有余额是否足够，
            self.allowances.insert((caller, spender), value);

            self.env().emit_event( Approval{
                owner : caller, spender, value,
            });
            Ok(())
        }

        /// 获取第一个账户授权第二个账户可使用的数量
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            // 或者 self.allowances.get(&(owner, spender)).copied().unwrap_or(0)
            // 因为 copied 后，就不需要解引用(*)了
            *self.allowances.get(&(owner, spender)).unwrap_or(&0)
        }

        /// 在授权(allowance)范围内，将指定账号的代币转到指定账号
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>{
            let caller = Self::env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to(Some(from), Some(to) , value)?;
            self.allowances.insert((from, to), allowance - value);

            Ok(())
        }

        /// 内部函数，用于从一个账户转账到另外一个账户
        fn transfer_from_to(&mut self, from: Option<AccountId>, to: Option<AccountId>, value:Balance) -> Result<()>{
            // 判断 from 账户是否有足够多的钱
            if let Some(from_account) = from {
                let from_balance = self.balance_of(from_account);
                if from_balance < value {
                    return Err(Error::InsufficientBalance)
                }
                self.balances.insert(from_account, from_balance - value);
            }
            if let Some(to_account) = to {
                //获取接受地址的余额
                let to_balance = self.balance_of(to_account);
                //再地址余额基础上加发送数量
                self.balances.insert(to_account, to_balance + value);
            }
            //发射转移事件
            self.env().emit_event( Transfer{
                from, to, value
            });
            Ok(())
        }


        /// 增发代币，只能创建者可以增发，增发的会直接转账给创建者，增发需要增加总供应量
        #[ink(message)]
        pub fn issue(&mut self, amount: Balance) -> Result<()>{
            //获取发送者
            let caller = Self::env().caller();
            //如果不是创建代币时的账户报错
            if caller != self.creator {
                return Err(Error::OnlyForCreator)
            }
            //获取当前发行总量
            let total_supply = self.total_supply();
            //发行总量增加
            self.total_supply = total_supply + amount;
            //调用者增发此次的代币数量
            self.transfer_from_to(None, Some(caller) , amount)?;
            Ok(())
        }

        /// 销毁代币并减少总供应量
        #[ink(message)]
        pub fn burn(&mut self, amount: Balance) -> Result<()>{
            //获取调用者
            let caller = Self::env().caller();
            //从发送者转移代币到接收地址
            self.transfer_from_to(Some(caller), None, amount)?;
            //获取当前发行总量
            let total_supply = self.total_supply();
            //减少总量
            self.total_supply = total_supply - amount;
            Ok(())
        }
    }
}
