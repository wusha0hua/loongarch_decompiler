echo "pub enum Opcode {" >> instruction.rs.bak
for name in `cat instructions`
do 
	echo -e "\t${name}," >> instruction.rs.bak
done

echo "}" >> instruction.rs.bak
