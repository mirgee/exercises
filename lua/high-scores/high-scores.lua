function table.slice(tbl, first, last, step)
	local sliced = {}

	for i = first or 1, last or #tbl, step or 1 do
		sliced[#sliced + 1] = tbl[i]
	end

	return sliced
end

return function(scoresList)
	local sortedScoresList = {}
	for i, val in ipairs(scoresList) do
		sortedScoresList[i] = val
	end
	table.sort(sortedScoresList, function(a, b)
		return a > b
	end)

	local function scores()
		return scoresList
	end

	local function latest()
		return scoresList[#scoresList]
	end

	local function personal_best()
		return sortedScoresList[1]
	end

	local function personal_top_three()
		return table.slice(sortedScoresList, 1, 3)
	end

	return {
		scores = scores,
		latest = latest,
		personal_best = personal_best,
		personal_top_three = personal_top_three,
	}
end
