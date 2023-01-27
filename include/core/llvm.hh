/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_LLVM_HH
#define DAL_CORE_LLVM_HH

#include <llvm-c/Core.h>
#include <llvm-c/Target.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Module.h>
#include <llvm/MC/SubtargetFeature.h>
#include <llvm/MC/TargetRegistry.h>
#include <llvm/Support/Host.h>
#include <llvm/Target/TargetMachine.h>
#include <llvm/Target/TargetOptions.h>
#include <string>

namespace dal::core {

void llvm_init();
std::string llvm_get_host_cpu_features();
llvm::TargetMachine* llvm_create_target_machine(
    const llvm::Target& target, const std::string& triple,
    const std::string& cpu, const std::string& features,
    llvm::Reloc::Model reloc_model, llvm::CodeGenOpt::Level opt_level);

}  // namespace dal::core

#endif  //DAL_CORE_LLVM_HH
