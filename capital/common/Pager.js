class Pager {
  constructor(page = 1, size = 10) {
    this.page = page;
    this.size = size;
  }
  setCount(total) {
    this.total = total;
    const pages = this.pages = Math.ceil(total / this.size);
    if (pages > 0 && this.page > pages) {
      this.page = pages;
    }
  }
}

module.exports = Pager;
