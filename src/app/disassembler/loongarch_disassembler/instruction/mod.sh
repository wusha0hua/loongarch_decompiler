for name in `ls`
do 
	if [ ${name} != "mod.sh" ] && [ ${name} != "opcode.sh" ] && [ ${name} != "opcode.rs" ]
	then
		echo "#[allow(unused)]" > ${name}
		echo "use super::*;" >> ${name}
	fi
done
