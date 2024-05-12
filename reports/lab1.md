# ch3 多道程序与分时多任务

## 总结

```
[kernel] Loading app_0
[kernel] PageFault in application, kernel killed it.
[kernel] Loading app_1
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_2
[kernel] IllegalInstruction in application, kernel killed it.
```

## 简答题

### q1

- sbi版本：RustSBI-QEMU Version 0.2.0-alpha.2

三个程序运行得到如下结果。

```
[kernel] Loading app_0                                       
[kernel] PageFault in application, kernel killed it.         
[kernel] Loading app_1                                       
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_2                                       
[kernel] IllegalInstruction in application, kernel killed it.
```

`app_0`尝试访问`0x0`陷入Trap之后`scause=7`即

以`ch2b_bad_address.rs`为例，在访问了`0x0`之后就跳转到Trap了

```
Breakpoint 2, 0x00000000804003ac in ?? ()
(gdb) x /10i $pc
=> 0x804003ac:
    sb  zero,0(zero) # 0x0
   0x804003b0:  auipc   a0,0x3
   0x804003b4:  addi    a0,a0,-688
   0x804003b8:  auipc   a1,0x3
   0x804003bc:  addi    a2,a1,-656
   0x804003c0:  li      a1,10
   0x804003c2:  auipc   ra,0x1
   0x804003c6:  jalr    -1304(ra)
   0x804003ca:  auipc   ra,0x1
   0x804003ce:  jalr    1378(ra)
(gdb) ni
0x0000000080200010 in __alltraps ()
```

### q2

1. 刚进入`_restore`时,`a0`存的是当前app的`TaskContext`的地址。`__restore`用于从系统调用中恢复到用户态；开始运行程序时跳转到程序开头
2. 从`TrapContext`中设置`sstatus`、`sepc`、`sscratch`寄存器。通过设置`sstatus`设定在`sret`之后要返回到的特权级，即切换回用户态。通过设置`sepc`设定`pc`的值，即返回用户态之后在哪条指令继续执行。通过设置`sscratch`设定返回后`sp`的值。
3. `x2`在L48已经处理，在返回前会将`sp`与`sscratch`互换。而`tp`寄存器一般应用不会用到。
4. 在执行前，`sscratech`存着用户栈顶地址，而`sp`是当前的内核栈顶地址，执行后两个值进行交换。
5. 发生在`sret`，该指令执行后，会根据先前在`sstatus`中设定的值切换到对应的状态，在这里是切换到用户态。
6. 在执行前，`sscratech`存着内核栈顶地址，而`sp`是当前的用户栈顶地址，执行后两个值进行交换。
7. 是在执行`ecall`的时候发生的。另外，外部中断也会使系统陷入S态

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与以下各位就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

2. 此外，我也参考了以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：


3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。






