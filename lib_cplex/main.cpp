#include <ilconcert/iloenv.h>
#include <ilcplex/ilocplexi.h>
#include <ratio>
#include <vector>
#include <chrono>


/*
  uint32_t *executar_cpp(uint32_t n_itens, uint32_t n_multas, const double
  *valores, const double *pesos, double capacidade, const uint32_t *multas_i,
  const uint32_t *multas_j, const double *valores_multas) {

  IloEnv env;
  IloModel mochila(env);
  IloCplex cplex(mochila);

  IloBoolVarArray x(env, n_itens);
  IloBoolVarArray v(env, n_itens);

  IloExpr somatorio_valor(env);
  for (int i = 0; i < n_itens; i++) {
  somatorio_valor += valores[i] * x[i];
  }

  IloExpr somatorio_multas(env);
  for (int k = 0; k < n_multas; k++) {
  somatorio_multas += v[k] * valores_multas[k];
  }

  mochila.add(IloMaximize(env, somatorio_valor - somatorio_multas));

  IloExpr peso_acumulado(env);
  for (int i = 0; i < n_itens; i++) {
  peso_acumulado += pesos[i] * x[i];
  }
  mochila.add(peso_acumulado <= capacidade);

  for (int k = 0; k < n_multas; k++) {
  IloExpr multa(env);
  multa += x[multas_i[k]] + x[multas_j[k]] - v[k];
  mochila.add(multa <= 1);
  }

  if (cplex.solve()) {
  IloNumArray sol(env, n_itens);
  cplex.getValues(sol, x);

  std::vector<int> itens;
  for (int i = 0; i < n_itens; i++) {
  if (sol[i] == 1) {
  itens.push_back(i);
  }
  }

  uint32_t *itens_ret = (uint32_t *)malloc(sizeof(uint32_t) * itens.size());
  for (int i = 0; i < itens.size(); i++) {
  itens_ret[i] = itens[i];
  }

  return itens_ret;
  }

  return (uint32_t *)calloc(sizeof(uint32_t), 1);
  }
*/

int main() {

  int n_itens;
  std::cin >> n_itens;

  double capacidade;
  std::cin >> capacidade;

  int n_multas;
  std::cin >> n_multas;

  std::vector<double> valores;
  std::vector<double> pesos;

  std::vector<int> multas_i;
  std::vector<int> multas_j;
  std::vector<double> valores_multas;

  for (int i = 0; i < n_itens; i++) {
    double pi;
    std::cin >> pi;
    valores.push_back(pi);
  }

  for (int i = 0; i < n_itens; i++) {
    double wi;
    std::cin >> wi;
    pesos.push_back(wi);
  }

  for (int k = 0; k < n_multas; k++) {
    int ik;
    int jk;
    double dk;
    std::cin >> ik;
    std::cin >> jk;
    std::cin >> dk;
    multas_i.push_back(ik);
    multas_j.push_back(jk);
    valores_multas.push_back(dk);
  }

  auto start = std::chrono::steady_clock::now();

  
  IloEnv env;
  IloModel mochila(env);
  IloCplex cplex(mochila);
  cplex.setOut(env.getNullStream());

  IloBoolVarArray x(env, n_itens);
  IloBoolVarArray v(env, n_multas);

  // Função objetivo
  IloExpr somatorio_valor(env);
  for (int i = 0; i < n_itens; i++) {
    somatorio_valor += valores[i] * x[i];
  }
  IloExpr somatorio_multas(env);
  for (int k = 0; k < n_multas; k++) {
    somatorio_multas += v[k] * valores_multas[k];
  }
  mochila.add(IloMaximize(env, somatorio_valor - somatorio_multas));
  
  // Restrições
  IloExpr peso_acumulado(env);
  for (int i = 0; i < n_itens; i++) {
    peso_acumulado += pesos[i] * x[i];
  }
  mochila.add(peso_acumulado <= capacidade);
  
  for (int k = 0; k < n_multas; k++) {
    IloExpr multa(env);
    multa += x[multas_i[k]] + x[multas_j[k]] - v[k];
    mochila.add(multa <= 1);
  }

  // Solução
  if (cplex.solve()) {

    auto diff = std::chrono::steady_clock::now() - start;
    std::cout << std::chrono::duration <double, std::milli> (diff).count() << "\n";
   
    IloNumArray sol(env, n_itens);
    cplex.getValues(sol, x);

    std::vector<int> itens;
    for (int i = 0; i < n_itens; i++) {
      if (sol[i] == 1) {
        itens.push_back(i);
      }
    }

    for(int item: itens) {
      std::cout << item << " ";
    }
    
  } else {
    std::cout << "[]";
  }

  return 0;
}
