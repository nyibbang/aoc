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

template<typename ForwardIt>
ForwardIt next_circular(ForwardIt begin, ForwardIt end, ForwardIt cur)
{
  if (cur == end || ++cur == end) {
    return begin;
  }
  return cur;
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
    if (*it == *next_circular(begin(input), end(input), it)) {
      std::cout << "Summing " << *it << "\n";
      sum += *it;
    }
  }
  std::cout << "Sum is " << sum << "\n";
}
