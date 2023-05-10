if __name__ == '__main__':
    abc = 'qwertyuiop[]\\' 

    with open('proxies.txt', 'r') as f:
        dirty_proxies = f.readlines()
        clean_proxies = f.readlines()

    for i in clean_proxies:
        for ii in i:
            if ii in abc:
                i = i.replace(ii, '')

    with open('proxies_dirty.txt', 'w') as f:
        for i in dirty_proxies:
            f.write(i)

    with open('proxies.txt', 'w') as f:
        for i in clean_proxies:
            f.write(i)
