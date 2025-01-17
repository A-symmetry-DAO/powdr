use ast::asm_analysis::{AnalysisASMFile, Machine, Rom};
use number::FieldElement;
use std::marker::PhantomData;

/// Generate the ROM for each machine based on its functions
/// This is very simple at the moment, because we only allow a single function. Therefore, the rom is the body of the function

pub fn generate_rom<T: FieldElement>(file: AnalysisASMFile<T>) -> AnalysisASMFile<T> {
    RomGenerator::default().generate(file)
}

#[derive(Default)]
struct RomGenerator<T> {
    marker: PhantomData<T>,
}

impl<T: FieldElement> RomGenerator<T> {
    fn generate(&self, file: AnalysisASMFile<T>) -> AnalysisASMFile<T> {
        AnalysisASMFile {
            machines: file
                .machines
                .into_iter()
                .map(|(name, m)| (name, self.generate_machine_rom(m)))
                .collect(),
        }
    }

    fn generate_machine_rom(&self, mut machine: Machine<T>) -> Machine<T> {
        machine.rom = machine.has_pc().then(|| {
            assert_eq!(
                machine.functions.len(),
                1,
                "only a single function is supported"
            );
            let main = machine.functions.first().unwrap();
            assert_eq!(
                main.name, "main",
                "the main function must be named \"main\""
            );
            assert!(
                main.params.inputs.params.is_empty(),
                "public inputs are not supported"
            );
            assert!(
                main.params
                    .outputs
                    .as_ref()
                    .map(|outputs| outputs.params.is_empty())
                    .unwrap_or(true),
                "public outputs are not supported"
            );

            // the rom is exactly the body of the main function!
            let rom = main.body.clone();

            Rom {
                statements: rom.statements,
                batches: None,
            }
        });
        machine
    }
}
