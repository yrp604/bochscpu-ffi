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

typedef void * ffi_hook_ctx;

ffi_hook_ctx bochscpu_hook_new(void);
void bochscpu_hook_delete(ffi_hook_ctx);

void bochscpu_hook_set_ctx(ffi_hook_ctx, void*);
void* bochscpu_hook_ctx(ffi_hook_ctx);

void bochscpu_hook_after_execution(ffi_hook_ctx, void (*)(void *, uint32_t, void *));
void bochscpu_hook_before_execution(ffi_hook_ctx, void (*)(void *, uint32_t, void *));
void bochscpu_hook_cache_cntrl(ffi_hook_ctx, void (*)(void *, uint32_t, uint32_t));
void bochscpu_hook_clflush(ffi_hook_ctx, void (*)(void *, uint32_t, gva_t, gpa_t));
void bochscpu_hook_cnear_branch_not_taken(void (*)(void *, uint32_t, gva_t));
void bochscpu_hook_cnear_branch_taken(ffi_hook_ctx, void (*)(void *, uint32_t, gva_t, gva_t));
void bochscpu_hook_exception(ffi_hook_ctx, void (*)(void *, uint32_t, uint32_t, uint32_t));
void bochscpu_hook_far_branch(ffi_hook_ctx, void (*)(void *, uint32_t, uint32_t, uint16_t, gva_t, uint16_t, gva_t));
void bochscpu_hook_hlt(ffi_hook_ctx, void (*)(void *, uint32_t));
void bochscpu_hook_hw_interrupt(ffi_hook_ctx, void (*)(void *, uint32_t, uint32_t, uint16_t, gva_t));
void bochscpu_hook_inp(ffi_hook_ctx, void (*)(void *, uint16_t, size_t));
void bochscpu_hook_inp2(ffi_hook_ctx, void (*)(void *, uint16_t, size_t, uint32_t));
void bochscpu_hook_interrupt(ffi_hook_ctx, void (*)(void *, uint32_t, uint32_t));
void bochscpu_hook_lin_access(ffi_hook_ctx, void (*)(void *, uint32_t, gva_t, gpa_t, size_t, uint32_t, uint32_t));
void bochcpu_hook_mwait(ffi_hook_ctx, void (*)(void *, uint32_t, gpa_t, size_t, uint32_t));
void bochscpu_hook_opcode(ffi_hook_ctx, void (*)(void *, uint32_t, void *, const uint8_t *, size_t, uint32_t, uint32_t));
void bochscpu_hook_outp(ffi_hook_ctx, void (*)(void *, uint16_t, size_t, uint32_t));
void bochscpu_hook_phy_access(ffi_hook_ctx, void (*)(void *, uint32_t, gpa_t, size_t, uint32_t, uint32_t));
void bochscpu_hook_prefetch_hint(ffi_hook_ctx, void (*)(void *, uint32_t, uint32_t, uint32_t, uint64_t));
// TODO: the rest