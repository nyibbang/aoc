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

int sum_difference_max_min_elements(int curr, const std::vector<int>& elems)
{
  const auto elemsit = minmax_element(begin(elems), end(elems));
  return curr + (*elemsit.second - *elemsit.first);
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

  const auto sum = accumulate(begin(input), end(input), 0, sum_difference_max_min_elements);
  std::cout << "Sum is " << sum << "\n";
}
