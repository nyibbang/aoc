#include <fstream>
#include <cmath>
#include <iostream>
#include <iterator>

int weight_to_fuel(int f) {
  return static_cast<int>(std::floor(static_cast<float>(f) / 3.f)) - 2;
}

int main() {
  std::ifstream in("input.txt");
  std::istream_iterator<int> it(in);
  decltype(it) end;

  unsigned long long fuel = 0;
  while (it != end) {
    auto module_weight = *it++;
    std::cerr << "module_weigth: " << module_weight << std::endl;
    auto module_fuel = weight_to_fuel(module_weight);
    std::cerr << "module_fuel: " << module_fuel << std::endl;
    fuel += module_fuel;
  }
  std::cout << fuel << std::endl;
}
