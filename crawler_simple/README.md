### Org from
```
org from: https://github.com/skerkour/black-hat-rust/tree/main/ch_05/crawler
```

### 改动
```
0.找到一个图片网站(https://www.vcg.com/creative-image/fengjing)做例子，爬取图片链接
1.练习select的用法(解析HTML)
2.复习原文章的代码(多线程并发, 同步, 通信等)
3.掌握爬取script网页的方法(webdirver)
```

## Install chromedriver

```shell
$ sudo apt install chromium-browser chromium-chromedriver
```

### Run chromedriver

```shell
$ chromedriver --port=4444 --disable-dev-shm-usage
```

### Usage
```
$ cargo build --release && ./target/release/crawler run --spider quotes
```

