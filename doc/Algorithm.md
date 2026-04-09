# MiniNEC Algorithm

## Introduction


[MiniNEC](https://apps.dtic.mil/sti/html/tr/ADA121535/index.html) is a software developed in 1981 by researchers at the Naval Ocean Systems Center (NOSC), USA. The project aimed to investigate whether it was possible to adapt techniques to create a reduced version of the NEC (Numerical Electromagnetics Code) antenna modeling software, enabling it to solve small problems on computers with limited resources. The resulting code was written in BASIC and could run on both an Apple II and a UNIVAC, each with only 64 kilobits of memory.

In MiniNEC, the antenna is described by wires, which in turn are divided in segments. The first simplification in MiniNEC is to assume constant current along each segment, and that all charges accumulate on the segment nodes. Those assumptions are the main difference when compared to NEC. In NEC,


* **A Abstração:** O fio é dividido em segmentos. O MiniNEC assume que a corrente é constante ao longo de cada segmento (pulso) e que as cargas se acumulam nos nós entre os segmentos.
* **A Equação Central:** Ele resolve a equação integral do campo elétrico (EFIE) na forma matricial:
  $$[Z][I] = [V]$$
  Onde $[Z]$ é a matriz de impedância mútua entre todos os segmentos, $[I]$ são as correntes desconhecidas e $[V]$ é o vetor de excitação (fontes).
* **Por que é mais fácil?** O cálculo dos elementos de $Z$ no MiniNEC envolve integrais mais simples (muitas vezes resolvidas por expansão em série ou fórmulas de campo próximo de fios finos) do que o tratamento de Kernel do NEC-2.


## Theory of operation




## Algorithmic Implementation