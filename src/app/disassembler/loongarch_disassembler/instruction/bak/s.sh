for name in `ls`
do
	if [ ${name} != "s.sh" ] && [ ${name} != "t" ] && [ ${name} != "sh.sh" ]
	then
		tac ${name} | sed  '0,/assembly_instruction/{s/assembly_instruction/assembly_instruction\n\t\n\t}\n\t\tassembly_instruction.label = Some(record.name.clone());\n\tif let Some(record) = symbol.get(\&address) {\n/}' | tac | sed  's/(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>)/(code: u32, address: usize, symbol: HashMap<usize, \&SymbolRecord>)/' | sed  's/assembly_instruction.opcode = Opcode::\(\w\{1,\}\);/assembly_instruction.opcode = Opcode::\1;\n\tassembly_instruction.address = address;\n/' | sed  's/value: 0,/value: 0,\n\t\tsymbol: None,/' 
	fi
done
