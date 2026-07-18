// Clean-room C++ fixture with a virtual hierarchy for vtable recovery.
#include <cstdio>

class Shape {
public:
    virtual ~Shape() {}
    virtual int area() const { return 0; }
    virtual const char *name() const { return "shape"; }
};

class Square : public Shape {
    int side;
public:
    explicit Square(int s) : side(s) {}
    ~Square() override {}
    int area() const override { return side * side; }
    const char *name() const override { return "square"; }
};

class Circle : public Shape {
    int radius;
public:
    explicit Circle(int r) : radius(r) {}
    ~Circle() override {}
    int area() const override { return 3 * radius * radius; }
    const char *name() const override { return "circle"; }
};

__attribute__((noinline))
int dispatch(const Shape *s) {
    // virtual dispatch through the vtable
    return s->area();
}

int main() {
    Square sq(4);
    Circle ci(2);
    const Shape *shapes[2] = { &sq, &ci };
    int total = 0;
    for (int i = 0; i < 2; ++i) {
        total += dispatch(shapes[i]);
        printf("%s=%d\n", shapes[i]->name(), shapes[i]->area());
    }
    return total & 0x7f;
}
