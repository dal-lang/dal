/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/llvm.hh>

namespace dal::core {

void initialize_llvm() {
  LLVMInitializeAllTargets();
  LLVMInitializeAllTargetMCs();
  LLVMInitializeAllAsmParsers();
  LLVMInitializeAllAsmPrinters();
  LLVMInitializeNativeTarget();
}

std::string get_host_triple() {
  std::string triple = llvm::sys::getDefaultTargetTriple();
  return triple;
}

std::unique_ptr<llvm::Module> new_module(const std::string& name) {
  auto ctx_ref = LLVMGetGlobalContext();
  auto& ctx = *reinterpret_cast<llvm::LLVMContext*>(ctx_ref);
  return std::make_unique<llvm::Module>(name, ctx);
}

std::string get_host_cpu_features() {
  llvm::SubtargetFeatures features;
  llvm::StringMap<bool> host_features;
  if (llvm::sys::getHostCPUFeatures(host_features)) {
    for (auto& f : host_features) {
      features.AddFeature(f.first(), f.second);
    }
  }
  return features.getString();
}

std::unique_ptr<llvm::TargetMachine> create_target_machine(
    const llvm::Target& target, const std::string& triple,
    const std::string& cpu, const std::string& features,
    llvm::Reloc::Model reloc_model, llvm::CodeGenOpt::Level opt_level) {
  llvm::TargetOptions options;
  auto m = target.createTargetMachine(triple, cpu, features, options,
                                      reloc_model, llvm::None, opt_level);
  return std::unique_ptr<llvm::TargetMachine>(m);
}

std::unique_ptr<llvm::IRBuilder<>> new_ir_builder(llvm::LLVMContext& ctx) {
  return std::make_unique<llvm::IRBuilder<>>(ctx);
}

}  // namespace dal::core