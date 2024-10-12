build:
	cargo build --release
	# 用于压缩可执行文件中的调试信息
	objcopy --compress-debug-sections ./target/release/message-notify ./main
	docker --debug build -t message-notify .
	rm -rf main

clean:
	cargo clean

push: build
	docker tag message-notify:latest registry.cn-hangzhou.aliyuncs.com/xinbaojian/rust-message-notify:latest
	docker push registry.cn-hangzhou.aliyuncs.com/xinbaojian/rust-message-notify:latest
	docker rmi registry.cn-hangzhou.aliyuncs.com/xinbaojian/rust-message-notify:latest
