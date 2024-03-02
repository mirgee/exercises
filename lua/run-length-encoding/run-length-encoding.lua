return {
	encode = function(s)
		if #s == 0 then
			return s
		end
		local res = ""
		local numSameEncountered = 1
		for i = 1, #s do
			if i > 1 then
				if s:sub(i - 1, i - 1) == s:sub(i, i) then
					numSameEncountered = numSameEncountered + 1
				elseif numSameEncountered > 1 then
					res = res .. numSameEncountered .. s:sub(i - 1, i - 1)
					numSameEncountered = 1
				else
					res = res .. s:sub(i - 1, i - 1)
					numSameEncountered = 1
				end
			end
		end
		if numSameEncountered == 1 then
			res = res .. s:sub(#s, #s)
		else
			res = res .. numSameEncountered .. s:sub(#s, #s)
		end
		return res
	end,
	decode = function(s)
		if #s == 0 then
			return s
		end
		local res = ""
		local prevNum = nil
		for i = 1, #s do
			local currChar = s:sub(i, i)
			local currNum = tonumber(currChar)
			-- We saw another number
			if currNum and prevNum then
				prevNum = currNum + prevNum * 10
			-- we saw no number with a previous number
			elseif prevNum and currNum == nil then
				res = res .. currChar:rep(prevNum)
				prevNum = nil
			elseif prevNum == nil and currNum == nil then
				res = res .. currChar
			else
				prevNum = currNum
			end
		end
		return res
	end,
}
