import re
import os

# CONFIGURATION
FILE_INPUT = "terminology.rs"  # Rename this to your actual 240,000-line FHIR file
TARGET_LINES_PER_FILE = 2400    # Desired length for each chunk

def split_fhir_file_massive(input_path, lines_per_file=2400):
    if not os.path.exists(input_path):
        print(f"Error: The file '{input_path}' does not exist.")
        return

    print("Reading input file...")
    with open(input_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    # 1. Process and clean ALL lines in the file first to fix the paths everywhere
    processed_lines = []
    for line in lines:
        if "self::super::types::Element" in line:
            line = line.replace("self::super::types::Element", "self::super::super::types::Element")
        if "super::types::Extension" in line:
            line = line.replace("super::types::Extension", "super::super::types::Extension")
        processed_lines.append(line)

    # 2. Isolate the header (the initial global attributes and imports)
    header = []
    code_start_idx = 0
    for idx, line in enumerate(processed_lines):
        if line.strip().startswith("#[derive") or line.strip().startswith("pub enum") or line.strip().startswith("pub struct"):
            code_start_idx = idx
            break
        header.append(line)

    code_lines = processed_lines[code_start_idx:]
    total_code_lines = len(code_lines)

    # Calculate how many files are needed based on the 2,400 line requirement
    estimated_files = total_code_lines // lines_per_file
    if estimated_files == 0:
        estimated_files = 1

    print(f"Total lines of code to split: {total_code_lines}.")
    print(f"Estimated generation: around {estimated_files} files with ~{lines_per_file} lines each.")

    # Pre-compile the regex for maximum speed on massive files
    type_regex = re.compile(r'\b(struct|enum)\b')

    chunks = []
    current_chunk = []

    for line_idx, line in enumerate(code_lines):
        # If the current chunk has reached the line target and we hit a safe type definition boundary
        if (len(current_chunk) >= lines_per_file and
            type_regex.search(line) and
            not line.strip().startswith(("//", "/*", "*"))):

            # Backtrack to capture any attributes/macros belonging to the type (e.g., #[derive], #[doc])
            lines_to_backtrack = 0
            while len(current_chunk) > 0:
                last_previous_line = current_chunk[-1].strip()
                if (last_previous_line.startswith("#") or
                    last_previous_line.startswith("///") or
                    last_previous_line.startswith("#[doc")):
                    lines_to_backtrack += 1
                    current_chunk.pop()
                else:
                    break

            # Save the completed chunk
            chunks.append(current_chunk)

            # Initialize the new chunk with the backtracked lines
            if lines_to_backtrack > 0:
                current_chunk = code_lines[line_idx - lines_to_backtrack : line_idx + 1]
            else:
                current_chunk = [line]
        else:
            current_chunk.append(line)

    if current_chunk:
        chunks.append(current_chunk)

    print(f"\nWriting {len(chunks)} files to disk...")

    # 3. Write the split files using the "terminology_000.rs" naming convention
    generated_modules = []
    for i, chunk in enumerate(chunks):
        file_name = f"terminology_{i:03d}.rs"
        generated_modules.append(f"terminology_{i:03d}")

        with open(file_name, "w", encoding='utf-8') as out:
            out.write("".join(header))
            out.write("\n")
            out.write("".join(chunk))

    print(f"Done! Created files from terminology_000.rs to terminology_{len(chunks)-1:03d}.rs")

    # 4. Generate the mod.rs file using internal 'mod' declarations
    print("Generating mod.rs file...")
    with open("mod.rs", "w", encoding='utf-8') as mod_file:
        mod_file.write("#![allow(non_camel_case_types)]\n\n")

        # Declare modules internally using 'mod' instead of 'pub mod'
        for mod in generated_modules:
            mod_file.write(f"mod {mod};\n")

        mod_file.write("\n// Re-export everything to the higher-level crate interface\n")
        for mod in generated_modules:
            mod_file.write(f"pub use {mod}::*;\n")

    print("Process completed successfully!")

if __name__ == "__main__":
    split_fhir_file_massive(FILE_INPUT, TARGET_LINES_PER_FILE)
