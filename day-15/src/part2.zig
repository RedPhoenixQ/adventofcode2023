const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = @embedFile("input.txt");

    const out = process(allocator, input);
    std.debug.print("OUT: {d}\n", .{out});
}

fn process(allocator: std.mem.Allocator, input: []const u8) usize {
    _ = allocator;

    var sum: usize = 0;

    var left: usize = 0;
    for (input, 0..) |c, i| {
        if (c == ',') {
            const hash = holiday_string_helper(input[left..i]);
            std.debug.print("Hash for \"{s}\" is {d} ({d}, {d})\n", .{ input[left..i], hash, left, i });
            sum += hash;
            left = i + 1;
        }
    }
    const hash = holiday_string_helper(input[left..]);
    std.debug.print("Hash for \"{s}\" is {d} ({d}, {d})\n", .{ input[left..], hash, left, input.len });
    sum += hash;

    return sum;
}

fn holiday_string_helper(string: []const u8) usize {
    var hash: usize = 0;
    for (string) |c| {
        hash += c;
        hash *= 17;
        hash %= 256;
    }
    return hash;
}

test "example" {
    const input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    try std.testing.expectEqual(@as(usize, 1320), process(std.testing.allocator, input));
}
