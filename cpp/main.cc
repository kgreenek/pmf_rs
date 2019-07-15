#include <chrono>
#include <iostream>
#include <vector>

std::vector<long double> compute_pmf(const std::vector<long double> &p_vector) {
  auto N = p_vector.size();
  std::vector<long double> pmf(N + 1, 0.0);
  pmf[0] = 1.0;

  for (int k = 0; k < N; k++) {
    for (int kk = k+1; kk > 0; kk--) {
      pmf[kk] += p_vector[k] * (pmf[kk-1] - pmf[kk]);
    }
    pmf[0] *= (1-p_vector[k]);
  }
  return pmf;
}

int main() {
  const auto N = 15000;
  const auto COUNT = 3;
  for (auto i = 0; i < 11; ++i) {
    auto p = i * 0.1;
    std::vector<long double> p_vector(N, p);
    auto total_ms = 0.0;
    for (auto j = 0; j < COUNT; ++j) {
      auto start_t = std::chrono::high_resolution_clock::now();
      auto pmf = compute_pmf(p_vector);
      auto end_t = std::chrono::high_resolution_clock::now();
      auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(end_t - start_t).count();
      //std::cout << "Loop " << j << " took " << ms << " ms" << std::endl;
      auto sum = static_cast<long double>(0.0);
      for (auto val : pmf) {
        sum += val;
      }
      if (1.0 - sum > 0.00001) {
        std::cout << "Invalid pmf!" << std::endl;
        exit(1);
      }
      total_ms += ms;
    }
    auto average_ms = static_cast<double>(total_ms) / static_cast<double>(COUNT);
    std::cout << "p " << p << " took " << average_ms << " ms" << std::endl;
  }
  return 0;
}
