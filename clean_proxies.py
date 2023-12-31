input_file_path = 'proxies.txt'
output_file_path = 'cleaned_proxies.txt'

with open(input_file_path, 'r') as input_file:
    proxy_list = input_file.read()

proxy_lines = proxy_list.strip().split('\n')

cleaned_proxies = [line.split()[0] for line in proxy_lines]

with open(output_file_path, 'w') as output_file:
    for proxy in cleaned_proxies:
        output_file.write(f"{proxy}\n")
