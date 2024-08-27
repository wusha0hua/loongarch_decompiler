names=(	LU12I_W
	LU12I_D
	PCADDI
	PCALAU12I
	PCADDU12I
	PCADDU18I)
for name in ${names[@]}
do
	file_name="${name,,}.rs"
	
	#sed -i 's/Generalregister/GeneralRegister/'	${file_name}
	#sed -i 's/Some(operand.clone())/Some(operand.clone());/'
	echo "#[allow(unused)]" > ${file_name}
	echo "use super::*;" >> ${file_name}
	echo "" >> ${file_name}
	echo "pub fn ${name,,}(code: u32) -> AssemblyInstruction {" >> ${file_name}
	echo -e "\tlet mut assembly_instruction = AssemblyInstruction::new();" >> ${file_name}
	echo -e "\tassembly_instruction.opcode = Opcode::${name};" >> ${file_name}
	echo "" >> ${file_name}
	echo -e "\tlet mut operand = Operand {" >> ${file_name}
	echo -e "\t\toperand_type: OperandType::GeneralRegister," >> ${file_name}
	echo -e "\t\tvalue: 0," >> ${file_name}
	echo -e "\t};" >> ${file_name}
	echo "" >> ${file_name}
	echo -e "\toperand.value = (code as usize) & ((1 << 5) - 1);" >> ${file_name}
	echo -e "\tassembly_instruction.operand1 = Some(operand.clone());" >> ${file_name}
	echo "" >> ${file_name}
	echo -e "\toperand.value = (code as usize >> 10) & ((1 << 16) - 1);" >> ${file_name}
	echo -e "\toperand.operand_type = OperandType::UnsignedImm;" >> ${file_name}
	echo -e "\tassembly_instruction.operand2 = Some(operand.clone());" >> ${file_name}
	#echo "" >> ${file_name}
	#echo -e "\toperand.value = (code as usize >> 10) & ((1 << 16) - 1);" >> ${file_name}
	#echo -e "\toperand.operand_type = OperandType::Offset;" >> ${file_name}
	#echo -e "\tassembly_instruction.operand3 = Some(operand.clone());" >> ${file_name}
	#echo "" >> ${file_name}
	#echo -e "\toperand.value = (code as usize >> 15) & ((1 << 5) - 1);" >> ${file_name}
	#echo -e "\tassembly_instruction.operand4 = Some(operand);" >> ${file_name}
	echo "" >> ${file_name}
	echo -e "\tassembly_instruction" >> ${file_name}
	echo "}" >> ${file_name}
done
