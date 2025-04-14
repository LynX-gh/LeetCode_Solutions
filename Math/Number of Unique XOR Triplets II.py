class Solution:
    def uniqueXorTriplets(self, nums: List[int]) -> int:
        max_val = max(nums)
        size = 1
        while size <= max_val:
            size <<= 1

        f = [0] * size
        for x in nums:
            f[x] = 1

        def fwht(a, invert):
            n = len(a)
            h = 1
            while h < n:
                for i in range(0, n, h*2):
                    for j in range(i, i+h):
                        u = a[j]
                        v = a[j+h]
                        a[j] = u+v
                        a[j+h] = u-v
                h <<= 1

            if invert:
                for i in range(n):
                    a[i] //= n

        fa = f[:]
        fwht(fa, False)

        for i in range(size):
            fa[i] = fa[i] * fa[i] * fa[i]

        fwht(fa, True)

        return sum(1 for v in fa if v > 0)
