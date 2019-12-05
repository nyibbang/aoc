#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <array>
#include <string>
#include <cmath>
#include <sstream>

using namespace std;

template<typename T, typename T2>
ostream& operator<<(ostream& os, const pair<T, T2>& p)
{
  os << "(" << p.first << ", " << p.second << ")";
  return os;
}

template<typename T>
ostream& operator<<(ostream& os, const vector<T>& tv)
{
  bool first = true;
  os << "[";
  for (const auto& v : tv)
  {
    if (!first)
      os << ", ";
    first = false;
    os << v;
  }
  os << "]";
  return os;
}

template<typename T, size_t N>
ostream& operator<<(ostream& os, const array<T, N>& tv)
{
  bool first = true;
  os << "[";
  for (const auto& v : tv)
  {
    if (!first)
      os << ", ";
    first = false;
    os << v;
  }
  os << "]";
  return os;
}

template<typename T, typename V>
ostream& operator<<(ostream& os, const map<T, V>& tv)
{
  bool first = true;
  os << "[";
  for (const auto& v : tv)
  {
    if (!first)
      os << ", ";
    first = false;
    os << v;
  }
  os << "]";
  return os;
}

template<typename T>
ostream& operator<<(ostream& os, const set<T>& tv)
{
  bool first = true;
  os << "[";
  for (const auto& v : tv)
  {
    if (!first)
      os << ", ";
    first = false;
    os << v;
  }
  os << "]";
  return os;
}

using banks_type = array<int, 16>;

auto read_input()
{
  banks_type res;
  for (auto& v : res)
    cin >> v;
  return res;
}

set<banks_type> configs;

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
  auto banks = read_input();

  int steps = 0;
  while (true)
  {
    cout << "Current state: " << banks << "\n";

    auto bank_it = max_element(begin(banks), end(banks));
    auto source_bank_it = bank_it;
    auto banks_count = *bank_it;
    while (banks_count > 0)
    {
      bank_it = next_circular(begin(banks), end(banks), bank_it);
      ++*bank_it;
      --banks_count;
      --*source_bank_it;
    }

    ++steps;
    auto res = configs.insert(banks);
    if (!res.second)
      break;
  }

  cout << steps << "\n";
}
