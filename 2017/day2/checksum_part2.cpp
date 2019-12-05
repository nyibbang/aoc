#include <iostream>
#include <vector>
#include <algorithm>
#include <sstream>

using namespace std;

int int_from_ascii(char c)
{
  if (c < '0' || c > '9') {
    return -1;
  }
  return c - '0';
}

template<typename ForwardIt>
std::pair<ForwardIt, ForwardIt> evenly_divisible_elements(ForwardIt begin, ForwardIt end)
{
  for (auto elemit = begin; elemit != end; ++elemit)
  {
    for (auto elemit2 = begin; elemit2 != end; ++elemit2)
    {
      if (elemit != elemit2 && *elemit % *elemit2 == 0)
      {
        std::cout << "Division: " << *elemit2 << " divides " << *elemit << "\n";
        return {elemit, elemit2};
      }
    }
  }
  return {end, end};
}

int sum_evenly_divisible_elements(int curr, const std::vector<int>& elems)
{
  const auto elemsit = evenly_divisible_elements(begin(elems), end(elems));
  return curr + (*elemsit.first / *elemsit.second);
}

int main(int, char**)
{
  string line;
  vector<vector<int>> input;
  while (getline(cin, line)) {
    vector<int> lineinput;
    istringstream linestream{line};
    int v = 0;
    while (linestream >> v) {
      lineinput.push_back(v);
    }
    input.push_back(lineinput);
  }

  const auto sum = accumulate(begin(input), end(input), 0, sum_evenly_divisible_elements);
  std::cout << "Sum is " << sum << "\n";
}
