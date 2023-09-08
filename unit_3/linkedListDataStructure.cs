using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace unit_3;


internal class Node
{
    internal int data;
    internal Node next;
    public Node (int d)
    {
        data = d;
        next = null;
    }
}

public class linkedListDataStructure
{
    public Node Head { get; set; }
    public int Length { get; set; }

    // is_empty() -> bool;
    // Returns true if the linked list is empty, otherwise returns false.
    public bool is_empty()
    {
        return Length == 0;
    }

    // pop() -> T;
    // Removes and returns the last item (element) in the linked list.
    public T pop()
    {

    }

    // peek() -> T;
    // Returns the last item (element) in the linked list without removing it.


    // prepend(item: T) -> void;
    // Adds an item to the beginning of the linked list.


    // append(item: T) -> void;
    // Adds an item to the end of the linked list.


    // insert_after(existing_item: T, new_item: T) -> void;
    // Inserts a new item after a specified existing item in the linked list.


    // delete(item: T) -> void;
    // Removes the first occurrence of the specified item from the linked list.


    // delete_at(index: int) -> void;
    // Removes the item at the specified index in the linked list.


    // delete_head() -> void;
    // Removes the first item (head) from the linked list.


    // size() -> int;
    // Returns the number of items in the linked list.
    public int size()
    {
        return Length;
    }

    // find(item: T) -> Node[T] or null;
    // Finds the first occurrence of the specified item in the linked list and returns its node. Returns null if the item is not found.


    // get(index: int) -> T;
    // Returns the item at the specified index in the linked list.


    // contains(item: T) -> bool;
    // Checks if the linked list contains the specified item.


    // index_of(item: T) -> int;
    // Returns the index of the first occurrence of the specified item in the linked list. Returns -1 if the item is not found.


    // reverse() -> void;
    // Reverses the order of the items in the linked list.


}
