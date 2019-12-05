#include <iostream>
#include <vector>

using namespace std;

int int_from_ascii(char c)
{
  if (c < '0' || c > '9') {
    return -1;
  }
  return c - '0';
}

template<typename RAIter>
RAIter halfway_around(RAIter begin, RAIter end, RAIter cur)
{
  const auto length = end - begin;
  const auto dist = length / 2;
  auto cur_dist = cur - begin + dist;
  if (cur_dist >= length) {
    cur_dist = cur_dist % length;
  }
  return begin + cur_dist;
}

int main(int, char**)
{
  char c = -1;
  std::vector<int> input;
  while (std::cin.get(c)) {
    const auto i = int_from_ascii(c);
    if (i != -1) {
      input.push_back(i);
    }
  }

  int sum = 0;
  for (auto it = begin(input); it != end(input); ++it)
  {
    if (*it == *halfway_around(begin(input), end(input), it)) {
      std::cout << "Summing " << *it << "\n";
      sum += *it;
    }
  }
  std::cout << "Sum is " << sum << "\n";
}
