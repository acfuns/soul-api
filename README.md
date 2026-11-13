## 启动
```shell
cargo r
```

## 项目构建
```
cargo b
./target/build/sou-api
```

安装 migration cli
```
cargo install sea-orm-cli@^2.0
```

## todo
1. 完成帖子的查找、创建、删除
2. 发帖可以先支持文字，后续支持图片和视频（文件数据库）
3. 点赞、收藏
4. 时间线服务（评论）

## docker-compose
由一系列的中间件服务构成
一键启动

docker compose up - d
