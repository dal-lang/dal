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

void initialize_llvm();

std::string get_host_triple();

std::unique_ptr<llvm::Module> new_module(const std::string& name);

std::string get_host_cpu_features();

std::unique_ptr<llvm::TargetMachine> create_target_machine(
    const llvm::Target& target, const std::string& triple,
    const std::string& cpu, const std::string& features,
    llvm::Reloc::Model reloc_model, llvm::CodeGenOpt::Level opt_level);

std::unique_ptr<llvm::IRBuilder<>> new_ir_builder(llvm::LLVMContext& ctx);

}  // namespace dal::core

#endif  //DAL_CORE_LLVM_HH
