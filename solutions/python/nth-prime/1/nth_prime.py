def prime(number):
    if number < 1:
        raise ValueError("there is no zeroth prime")
    primes = []
    candidate = 2
    while len(primes) < number:
        is_prime = True
        for p in primes:
            if p * p > candidate:
                break
            if candidate % p == 0:
                is_prime = False
                break
        if is_prime:
            primes.append(candidate)
        candidate += 1

    return primes[-1]
