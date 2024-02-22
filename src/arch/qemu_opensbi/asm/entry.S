
# 禁止压缩指令
.option norvc
.section .text.init
.global _start
_start:

.option push
# 根据手册说明，意思是不允许链接器松弛（？）
.option norelax
	# 不同核心栈区分开
	la		sp, _stack_end
	# li		t0, 0x20000
	# csrr	a0, mhartid
	# mul		t0, t0, a0
	# sub		sp, sp, t0
	# 让其它（非0号）硬件线程挂起，跳转至 3
	# csrr	t0, mhartid
	# csrw	satp, zero
	# bnez	t0, loop_wait

	call    kernel_init

loop_wait:
    wfi
    j   loop_wait
