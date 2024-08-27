function parse_arg(str)
	l = strlen(str)
	x = 0
	i = 0
	while(i < l) do
		c = strat(str, i)
		if(c == 37) then
			x = x + 1
		end
		i = i + 1
	end
	return x + 1
end

str = _arg_
_res_ = parse_arg(str)
_store_ = false

