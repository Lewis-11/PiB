# Project in Bioinformatics

Methods to present and implement
--------------------------------

Define the problem of computing an MSA using column-based SP score. This might also include an introduction to pairwise alignment.

Describe and implement the dynamic programming based algorithm for computing an exact SP-MSA of k sequences of length n in time O(n^k).

Describe and implement Gusfield's 2-approximation algorithm for computing an SP-MSA of k sequences of length n that has a score that is at most twice the score of an optimal SP-MSA in time O(k^2 n^2).

Extend the idea of Gusfields algorithm to use an MST as guide tree, where alignments are merged in the order according to how edges are added to the MST by the MST-construction algorithm. There are several MST construction algorithms, fx Prim's or Kruskal's algorithm. Describe and implement one approach.

Make experiments with the implemented algorithms that examine their running time in practice.

Make experiments and (simulated) datasets to examine the score of the MSAs computed by the different algorithms. This could involve finding/making a program for simulating sequence evolution in a simple model.

Material
--------

* Gusfield1997_MSA.pdf

Chapter 14 from the book "D. Gusfield. Algorithms on Strings, Trees, and Sequences: Computer Science and Computational Biology. Cambridge University Press, 1997.". 

The chapter introduces MSA. In particular 14.6.1 is about an exact algorithm, and 14.6.2 is about the 2-approximation algorithm.

* Gusfield1993_Article_EfficientMethodsForMultipleSeq.pdf

The original paper by Gusfield that presents the approximation algorithm from section 14.6.2 above.
Efficient methods for multiple sequence alignment with guaranteed error bounds
Dan Gusfield 
Bulletin of Mathematical Biology volume 55, pages141–154(1993)
https://link.springer.com/article/10.1007/BF02460299

* MSA.pdf
* SP-MSA-Approx.pdf
* GlobalPairwiseAlignment.pdf

Slides from the class 'Algorithms in Bioinformatics' about pairwise alignment, MSA, and Gusfield's approximation algorithm.

* OhleBusch2013_Chap8_MSA.pdf
* OhleBusch2013_Chap8_PairwiseAlignment.pdf

Chapters from the book "E. Ohlebusch. Bioinformatics Algorithms: Sequence Analysis, Genome Rearrangements, and Phylogenetic Reconstruction. Oldenbusch Verlag, 2013." about the above.

* MST_Chap23_IntroductionToAlgorithms.pdf

Chapter from the book "Introduction to Algorithms" about MSTs. In this project it is probably most convenient to use Prim's algorithm in the O(|V|^2) time version hinted at in exercise 23.2-2 because its running time is better than the more complicated implementation for complete graphs, where |E| = |V|^2.
