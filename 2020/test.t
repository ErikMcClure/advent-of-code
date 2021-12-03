local state = {}
local turn = 0
local start = {0, 14, 1, 3, 7, 9}
local function runturn(num)
  local res = 0
  if state[num] then
    res = turn - state[num]
  end
  state[num] = turn
  turn = turn + 1
  return res
end
local last = 0
for _, v in ipairs(start) do
  last = runturn(v)
  print(v)
end

for i = #start, 29999998 do
  -- print(last)
  last = runturn(last)
end
print(last)