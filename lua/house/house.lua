local house = {}

local sentences = {
	"the house that Jack built",
	"the malt",
	"the rat",
	"the cat",
	"the dog",
	"the cow with the crumpled horn",
	"the maiden all forlorn",
	"the man all tattered and torn",
	"the priest all shaven and shorn",
	"the rooster that crowed in the morn",
	"the farmer sowing his corn",
	"the horse and the hound and the horn",
}

local verbs = {
	"lay in",
	"ate",
	"killed",
	"worried",
	"tossed",
	"milked",
	"kissed",
	"married",
	"woke",
	"kept",
	"belonged to",
}

house.verse = function(which)
	local res = "This is " .. sentences[which]
	for i = which - 1, 1, -1 do
		if i == which - 1 then
			res = res .. "\n"
		end
		res = res .. "that " .. verbs[i] .. " " .. sentences[i]
		if i ~= 1 then
			res = res .. "\n"
		end
	end
	return res .. "."
end

house.recite = function()
	local res = ""
	for i = 1, #sentences do
		res = res .. house.verse(i)
		if i ~= #sentences then
			res = res .. "\n"
		end
	end
	return res
end

return house
