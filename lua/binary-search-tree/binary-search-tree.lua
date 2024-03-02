local BinarySearchTree = {}

function BinarySearchTree:new(value)
	local newObj = {
		value = value,
		left = nil,
		right = nil,
	}
	return setmetatable(newObj, { __index = self })
end

function BinarySearchTree:insert(value)
	if value <= self.value then
		if self.left then
			self.left:insert(value)
		else
			self.left = BinarySearchTree:new(value)
		end
	else
		if self.right then
			self.right:insert(value)
		else
			self.right = BinarySearchTree:new(value)
		end
	end
end

function BinarySearchTree:from_list(list)
	if #list == 0 then
		error("Empty list")
	end
	local tree = BinarySearchTree:new(list[1])
	for i = 2, #list do
		tree:insert(list[i])
	end
	return tree
end

local function dfs(node, list)
	if node == nil then
		return
	end
	dfs(node.left, list)
	table.insert(list, node.value)
	dfs(node.right, list)
end

function BinarySearchTree:values()
	local values = {}
	dfs(self, values)
	local pos = 0
	return function()
		pos = pos + 1
		return values[pos]
	end
end

return BinarySearchTree
