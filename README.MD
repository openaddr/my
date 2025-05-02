godot-rust实现的第一个小游戏,

项目已完成,可直接构建, 需要安装rust(使用nightly通道), godot4.4

执行cargo build后,在godot中启动项目即可.

涉及基础知识点:
1. godot-rust的基本用法
   1. 如何用Rust编写Godot节点
   2. Godot内置信号如何绑定到Rust方法
   3. OnReady<Gd<PackedScene>>类似方法加载预置的子节点
2. gedxt-coroutines,在godot中使用rust的协程
3. godot知识:
   1. 使用CharacterBody2D节点方便地控制主角移动
   2. 使用CollisionShape2D节点设置碰撞体
   3. 使用Area2D节点检测碰撞
   4. 使用Sprite2D节点设置角色的图片
   5. 使用AnimatedSprite2D节点设置角色的动画
   6. 使用VisibleOnScreenNotifier2D节点检测物体超出屏幕
   7. 使用Timer节点计时,循环执行代码
   8. 使用AudioStreamPlayer节点播放音频
