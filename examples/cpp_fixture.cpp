// Original bounded fixture: one explicitly instrumented allocation, not all runtime allocations.
extern "C" int printf(const char*, ...);
extern "C" unsigned long strtoul(const char*, char**, int);

int main(int argc, char** argv) {
    if (argc != 2) return 2;
    char* end = nullptr;
    const unsigned long mb = strtoul(argv[1], &end, 10);
    if (*end || mb < 1 || mb > 64) return 2;
    const unsigned long size = mb * 1024 * 1024;
    auto* data = new unsigned char[size];
    for (unsigned long i = 0; i < size; ++i) data[i] = 1;
    volatile unsigned char* visible = data;
    const unsigned long checksum = visible[0] + visible[size - 1];
    printf("{\"allocation_calls\":1,\"allocated_bytes\":%lu,\"peak_live_bytes\":%lu,\"checksum\":%lu}\n",
           size, size, checksum);
    delete[] data;
    return 0;
}
