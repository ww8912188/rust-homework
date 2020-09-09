const Pager = require('../common/Pager');
class BasicService {
  constructor(model) {
    this.model = model;
  }
  async findById() {
    return this.model.findById(...arguments);
  }
  async findOne() {
    return this.model.findOne(...arguments);
  }
  async find() {
    return this.model.find(...arguments);
  }
  async findOneAndUpdate() {
    return this.model.findOneAndUpdate(...arguments);
  }
  async findByPage(query, fields, pager, sort) {
    pager = pager || new Pager();
    const count = await this.model.countDocuments(query);
    pager.page -= 0;
    pager.size -= 0;
    pager.setCount(count);
    return {
      data: await this.model.find(query, fields).sort(sort).skip((pager.page - 1) * pager.size).limit(pager.size).lean(),
      pager
    };
  }
  async updateById(_id, update, findNew) {
    return this.update({ _id }, update, findNew);
  }
  async update(query, update, findNew) {
    if (findNew) {
      return this.model.findOneAndUpdate(query, update, { new: true });
    } else {
      return this.model.updateOne(query, update);
    }
  }
  async deleteOne() {
    return this.model.deleteOne(...arguments);
  }
  async deleteMany() {
    return this.model.deleteMany(...arguments);
  }
  async updateMany() {
    return this.model.updateMany(...arguments);
  }
  async updateOne() {
    return this.model.updateOne(...arguments);
  }
  async save() {
    return new this.model(...arguments).save();
  }
  async aggregate() {
    return this.model.aggregate(...arguments);
  }
}

module.exports = BasicService;
