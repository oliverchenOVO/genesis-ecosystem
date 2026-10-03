# v5 morphology model

The original 14 bounded haploid loci remain at indices 0–13. Eight appended loci (14–21) expose a structured `MorphologyGenome`: segmentation, aspect, appendages, mouth, armor, sensory, locomotion, storage. All alleles are integers 0–1000. Existing independent reproduction/mutation streams select each parental allele and mutate without any environmental fitness direction. Small shifts ±60 dominate; 4% of mutated loci use ±300; 1% redraw 0–1000. Bounds clamp deterministically. RNG algorithm/version is unchanged.

Founder segmentation is 175–225, thus every founder has one segment. Descendants may cross 250 through inherited random variation; there is no time/generation unlock. Three mouth intervals are grazer 0–332, crusher 333–665 and piercer 666–1000. Founders have standing dietary variation, not predefined species roles. Mouth type alone never creates a reproductive barrier.

All arithmetic below is integer division, with body size B from original locus0:

- segments S = 1 + segmentation/250 (1–5); aspect = 500+aspect allele (thousandths); appendages A = appendage allele/125 (0–8).
- armor R = armor allele×600/1000 (0–600, thousandths); mass M = B×100+(S−1)×150+R/2.
- complexity C = (S−1)×80+A×12+R/6+sensory/12+locomotion allele/16+storage/16.
- maintenance = 1+C/90; basal metabolism adds this to original metabolic/body costs. Sensory range =16+original vision allele/24+sensory/40.
- locomotion expression L=500+allele. Speed =max(1, original base speed×L/(1000+M/4+R/2)). Movement coefficient=max(50,100+M/10+R/2+abs(aspect−1000)/5−locomotion allele/8−A×4).
- capacity=1200+original capacity allele×2+M×3+storage×2. Construction cost K=400+M+C×2. Reproduction threshold=max(original fractional capacity threshold,K).
- reproduction maturity 60+C/4 ticks and cooldown80+C/4. Each parent spends energy/4+K/2; combined offspring energy subtracts both parents' C and is shared across offspring, bounded by each child's capacity. This models construction loss, not optional juvenile growth.

More segments/storage yield larger capacity and bite capacity, and mass reduces thermal energy loss; maintenance, movement, construction and reproductive delay oppose those benefits. Armor reduces attack damage while increasing mass, movement coefficient and construction cost. Sensors improve range at a maintenance cost. Appendages/locomotion improve speed at complexity/construction cost. No structural investment is free.

The representation and formulas are artificial-life rules, not claims of molecular or real-world biology. Optional juvenile growth is deferred to Phase2.1.

## Revision2 readiness and mechanical feeding

The selected D candidate retains the original capacity, maturity, cooldown, maintenance, movement and parental construction payments. Reproductive readiness uses reserve=1200+original capacity allele*2+B*300, threshold=max(reserve*(55+original threshold allele/40)/100,K). Additional morphology/storage capacity is usable reserve rather than a proportional second readiness penalty; actual construction K is still paid. No phenotype-label bonus or global energy subsidy exists.

Crusher resource efficiency=65+hardness/12+hardness*clamp(bite_capacity-hardness/20,0,20)/50. This is a bounded bite-dependent payoff for processing hard resources; at hardness0 it adds nothing, and insufficient bite capacity adds nothing. Grazer125-hardness/12 and Piercer35 are unchanged. Feeding quota25 remains unchanged because measured intake was supply-limited, not quota-limited.

Mating retains the floored mean22-locus distance<=220 and the original14-locus per-locus distance<=400. The structural8 loci contribute to global distance without an additional400 cliff. Diet remains in the original14-locus barrier; removing that barrier was separately tested and rejected after5/8 natural world extinctions. Mutation distributions and RNG consumption rules are unchanged. See PHASE2_FINALIZATION.md for bounded paired comparisons and retained failures.
