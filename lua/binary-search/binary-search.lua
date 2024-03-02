local function binsearch(array, target, start, finish)
	if finish < start then
		return -1
	end
	local len = finish - start + 1
	local mid = math.floor(len / 2) + start
	local midVal = array[mid]
	if midVal == target then
		return mid
	elseif midVal < target then
		return binsearch(array, target, mid + 1, finish)
	elseif midVal > target then
		return binsearch(array, target, start, mid - 1)
	end
end

return function(array, target)
	local arrayLen = #array
	if arrayLen == 0 then
		return -1
	end
	return binsearch(array, target, 1, arrayLen)
end
