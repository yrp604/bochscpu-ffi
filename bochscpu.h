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

// TODO the rest of these
