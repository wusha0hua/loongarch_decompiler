files=(clo_w.rs clz_w.rs
 cto_w.rs
 ctz_w.rs
 clo_d.rs
 clz_d.rs
 cto_d.rs
 ctz_d.rs
 revb_2h.rs
 revb_4h.rs
 revb_2w.rs
 revb_d.rs
 revh_2w.rs
 revh_d.rs
 bitrev_4b.rs
 bitrev_8b.rs
 bitrev_w.rs
 bitrev_d.rs ext_w_h.rs ext_w_b.rs)

echo > t
for name in ${files[@]}
do
	sed -i 's/operand.value = (code \& ((1 << 5) - 1)) as usize;/operand.value = (code \& ((1 << 5) - 1)) as usize;\n\tassembly_instruction.regs_write.push(Register::GR(operand.value));/' ${name}  
	sed -i 's/operand.value = (code as usize >> 5) \& ((1 << 5) - 1);/operand.value = (code as usize >> 5) \& ((1 << 5) - 1);\n\tassembly_instruction.regs_read.push(Register::GR(operand.value))/' ${name}
done


#for name in `ls`
#do
#	out=`echo ${name} | grep "^am"`
#	if [ -n "{out}" ]
#	then
#		echo "${name}"
#	fi
#	#if [ ${name} != "s.sh" ] && [ ${name} != "t" ] && [ ${name} != "sh.sh" ]
#	#then
#	#	echo ${name}
#	#fi
#done

#for name in `ls | grep "^am"`
#do
	#sed -i 's/operand.value = (code as usize) \& ((1 << 5) - 1);/operand.value = (code as usize) \& ((1 << 5) - 1);\n\tassembly_instruction.regs_write.push(Register::GR(operand.value));/' ${name} 
	#sed -i 's/operand.value = (code as usize >> 5) \& ((1 << 5) - 1);/operand.value = (code as usize >> 5) \& ((1 << 5) - 1);\n\tassembly_instruction.regs_read.push(Register::GR(operand.value));/' ${name} 
	#sed -i 's/operand.value = (code as usize >> 10) \& ((1 << 5) - 1);/operand.value = (code as usize >> 10) \& ((1 << 5) - 1);\n\tassembly_instruction.regs_read.push(Register::GR(operand.value));/' ${name}

#done
