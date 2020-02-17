#include <stdint.h>

typedef uint64_t gpa_t;
typedef uint64_t gva_t;
typedef uint8_t* hva_t;

typedef void* hook_t;
typedef void* cpu_t;

///
/// memory
///

void bochscpu_mem_page_insert(gpa_t, hva_t);
void bochscpu_mem_page_remove(gpa_t);

void bochscpu_mem_missing_page(void (*)(gpa_t));

hva_t bochscpu_mem_translate_phy(gpa_t);
gpa_t bochscpu_mem_translate_virt(gva_t);

void bochscpu_mem_read_phy(gpa_t, hva_t, size_t);
void bochscpu_mem_write_phy(gpa_t, hva_t, size_t);

int bochscpu_mem_read_virt(gpa_t, gpa_t, hva_t, size_t);
int bochscpu_mem_write_virt(gpa_t, gpa_t, hva_t, size_t);

///
/// cpu
///

cpu_t bochscpu_cpu_new(uint32_t);
void bochscpu_cpu_delete(cpu_t);

///
/// hooks
///

hook_t bochscpu_hook_new(void);
void bochscpu_hook_delete(hook_t);

void bochscpu_hook_set_ctx(hook_t, void*);
void* bochscpu_hook_ctx(hook_t);

void bochscpu_hook_reset(hook_t, void (*)(void *, uint32_t, uint32_t));
void bochscpu_hook_hlt(hook_t, void (*)(void *, uint32_t));
void bochscpu_hook_mwait(hook_t, void (*)(void *, uint32_t, gpa_t, size_t, uint32_t));

void bochscpu_hook_cnear_branch_taken(hook_t, void (*)(void *, uint32_t, gva_t, gva_t));
void bochscpu_hook_cnear_branch_not_taken(hook_t, void (*)(void *, uint32_t, gva_t));
void bochscpu_hook_ucnear_branch(hook_t, void (*)(void *, uint32_t, uint32_t, uint64_t, uint64_t));
void bochscpu_hook_far_branch(hook_t, void (*)(void *, uint32_t, uint32_t, uint16_t, gva_t, uint16_t, gva_t));

void bochscpu_hook_tlb_cntrl(hook_t, void (*)(void *, uint32_t, gpa_t));
void bochscpu_hook_cache_cntrl(hook_t, void (*)(void *, uint32_t, uint32_t));
void bochscpu_hook_prefetch_hint(hook_t, void (*)(void *, uint32_t, uint32_t, uint32_t, uint64_t));
void bochscpu_hook_clflush(hook_t, void (*)(void *, uint32_t, gva_t, gpa_t));

void bochscpu_hook_after_execution(hook_t, void (*)(void *, uint32_t, void *));
void bochscpu_hook_before_execution(hook_t, void (*)(void *, uint32_t, void *));
void bochscpu_hook_repeat_iteration(hook_t, void (*)(void *, uint32_t, void *));

void bochscpu_hook_opcode(hook_t, void (*)(void *, uint32_t, void *, const uint8_t *, size_t, uint32_t, uint32_t));
void bochscpu_hook_interrupt(hook_t, void (*)(void *, uint32_t, uint32_t));
void bochscpu_hook_exception(hook_t, void (*)(void *, uint32_t, uint32_t, uint32_t));
void bochscpu_hook_hw_interrupt(hook_t, void (*)(void *, uint32_t, uint32_t, uint16_t, gva_t));

void bochscpu_hook_inp(hook_t, void (*)(void *, uint16_t, size_t));
void bochscpu_hook_inp2(hook_t, void (*)(void *, uint16_t, size_t, uint32_t));
void bochscpu_hook_outp(hook_t, void (*)(void *, uint16_t, size_t, uint32_t));

void bochscpu_hook_lin_access(hook_t, void (*)(void *, uint32_t, gva_t, gpa_t, size_t, uint32_t, uint32_t));
void bochscpu_hook_phy_access(hook_t, void (*)(void *, uint32_t, gpa_t, size_t, uint32_t, uint32_t));

void bochscpu_hook_wrmsr(hook_t, void (*)(void *, uint32_t, uint32_t, uint64_t));

void bochscpu_hook_vmexit(hook_t, void (*)(void *, uint32_t, uint32_t, uint64_t));
