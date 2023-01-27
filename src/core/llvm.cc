/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/llvm.hh>

namespace dal::core {

void llvm_init() {
  LLVMInitializeAllTargets();
  LLVMInitializeAllTargetMCs();
  LLVMInitializeAllAsmParsers();
  LLVMInitializeAllAsmPrinters();
  LLVMInitializeNativeTarget();
}

std::string llvm_get_host_cpu_features() {
  llvm::SubtargetFeatures features;
  llvm::StringMap<bool> host_features;
  if (llvm::sys::getHostCPUFeatures(host_features)) {
    for (auto& f : host_features) {
      features.AddFeature(f.first(), f.second);
    }
  }
  return features.getString();
}

llvm::TargetMachine* llvm_create_target_machine(
    const llvm::Target& target, const std::string& triple,
    const std::string& cpu, const std::string& features,
    llvm::Reloc::Model reloc_model, llvm::CodeGenOpt::Level opt_level) {
  llvm::TargetOptions options;
  auto m = target.createTargetMachine(triple, cpu, features, options,
                                      reloc_model, llvm::None, opt_level);
  return m;
}

}  // namespace dal::core