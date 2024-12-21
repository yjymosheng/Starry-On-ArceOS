# img适配

修改内容: 

    1. /home/mosheng/github/Starry-On-ArceOS/Cargo.toml 添加axfs
        
    2. /home/mosheng/github/Starry-On-ArceOS/Makefile 中在run 里面添加 "NET=y BLK=y" 启用网络和块设备

    3. 从Starry-Newcopy过来build_img.sh, 修改97与104行有关cp的测试文件位置为 apps/riscv64

    4. .arceos/modules/axdriver/Cargo.toml default 添加"block","virtio-blk","virtio"

    5. apps/riscv64添加testcase_list 文件
    
    6. src/loader.rs 改为从axfs::api::read 读入

    7. src/main.rs 的loader::list_app()无用了,注释掉


syscall实现:

    1. 原arceos有许多已实现的syscall位于/home/mosheng/github/Starry-On-ArceOS/.arceos/api 可以直接调用(不过耦合太深,先选择性忽略吧)
    
