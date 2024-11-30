import os
import binwalkpy

file_path = os.path.abspath("./MR90-V1.0.2.38_1.0.38.chk")
output_path = os.path.abspath("./extracted")

# Test file scanning
results = binwalkpy.scan_file(file_path)
for result in results:
    print(result)

# Test file extraction
results = binwalkpy.extract(file_path, include=["png", "html"], full_search=True)
for extract_result in results:
    print(extract_result)