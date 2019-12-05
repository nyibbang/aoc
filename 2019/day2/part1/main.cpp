#include <iostream>
#include <iterator>
#include <vector>
#include <fstream>
#include <cassert>

template<typename Fn>
std::size_t opcode_3pos(std::vector<int>& codes, std::size_t index, Fn op) {
  auto in_pos1 = codes.at(++index);
  auto in_pos2 = codes.at(++index);
  auto out_pos = codes.at(++index);
  codes.at(out_pos) = op(codes.at(in_pos1), codes.at(in_pos2));
  return ++index;
}

std::size_t opcode_1(std::vector<int>& codes, std::size_t index) {
  assert(codes.at(index) == 1);
  return opcode_3pos(codes, index, std::plus<int>{});
}

std::size_t opcode_2(std::vector<int>& codes, std::size_t index) {
  assert(codes.at(index) == 2);
  return opcode_3pos(codes, index, std::multiplies<int>{});
}

int main() {
  std::ifstream in("input.txt");

  std::vector<int> codes;
  while(in && !in.eof()) {
    int code = 0;
    in >> code;
    codes.push_back(code);
    if (in.eof())
      break;
    in.ignore(std::numeric_limits<std::streamsize>::max(), ',');
  }

  if (codes.size() > 2) {
    codes.at(1) = 12;
  }
  if (codes.size() > 3) {
    codes.at(2) = 2;
  }

  std::size_t index = 0;
  while(index < codes.size()) {
    switch(codes.at(index)) {
      case 1:
        index = opcode_1(codes, index);
        break;
      case 2:
        index = opcode_2(codes, index);
        break;
      case 99:
        std::cout << codes.at(0) << std::endl;
        return 0;
    }
  }

}
