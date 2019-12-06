#include <stdint.h>

typedef uint64_t gpa_t;
typedef uint64_t gva_t;
typedef uint8_t* hva_t;

///
/// memory
///

void bochscpu_mem_add_page(gpa_t, hva_t);
void bochscpu_mem_del_page(gpa_t);

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

///
/// hooks
///

void bochscpu_hook_after_execution(void (*)(uint32_t, void *));
void bochscpu_hook_after_execution_clear(void);

void bochscpu_hook_before_execution(void (*)(uint32_t, void *));
void bochscpu_hook_before_execution_clear(void);

void bochscpu_hook_cache_cntrl(void (*)(uint32_t, uint32_t));
void bochscpu_hook_cache_cntrl_clear(void);

void bochscpu_hook_clflush(void (*)(uint32_t, gva_t, gpa_t));
void bochscpu_hook_clflush_clear(void);

void bochscpu_hook_cnear_branch_not_taken(void (*)(uint32_t, gva_t));
void bochscpu_hook_cnear_branch_not_taken_clear(void);

void bochscpu_hook_cnear_branch_taken(void (*)(uint32_t, gva_t, gva_t));
void bochscpu_hook_cnear_branch_taken_clear(void);

void bochscpu_hook_exception(void (*)(uint32_t, uint32_t, uint32_t));
void bochscpu_hook_exception_clear(void);

void bochscpu_hook_far_branch(void (*)(uint32_t, uint32_t, uint16_t, gva_t, uint16_t, gva_t));
void bochscpu_hook_far_branch_clear(void);

void bochscpu_hook_hlt(void (*)(uint32_t));
void bochscpu_hook_hlt_clear(void);

void bochscpu_hook_hw_interrupt(void (*)(uint32_t, uint32_t, uint16_t, gva_t));
void bochscpu_hook_hw_interrupt(void);

void bochscpu_hook_inp(void (*)(uint16_t, size_t));
void bochscpu_hook_inp_clear(void);

void bochscpu_hook_inp2(void (*)(uint16_t, size_t, uint32_t));
void bochscpu_hook_inp2_clear(void);

void bochscpu_hook_interrupt(void (*)(uint32_t, uint32_t));
void bochscpu_hook_interrupt_clear(void);

void bochscpu_hook_lin_access(void (*)(uint32_t, gva_t, gpa_t, size_t, uint32_t, uint32_t));
void bochscpu_hook_lin_access_clear(void);

void bochcpu_hook_mwait(void (*)(uint32_t, gpa_t, size_t, uint32_t));
void bochcpu_hook_mwait_clear(void);

void bochscpu_hook_opcode(void (*)(uint32_t, void *, const uint8_t *, size_t, uint32_t, uint32_t));
void bochscpu_hook_opcode_clear(void);

void bochscpu_hook_outp(void (*)(uint16_t, size_t, uint32_t));
void bochscpu_hook_outp_clear(void);

void bochscpu_hook_phy_access(void (*)(uint32_t, gpa_t, size_t, uint32_t, uint32_t));
void bochscpu_hook_phy_access_clear(void);


void bochscpu_hook_clear(void);

// TODO the rest of these
