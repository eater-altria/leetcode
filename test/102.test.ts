import { describe, expect, test } from '@jest/globals';
import { TreeNode, levelOrder } from '../src/102';

describe('102 Test', () => {
  test('Case 1', () => {
    const Node15 = new TreeNode(15, null, null);
    const Node7 = new TreeNode(7, null, null);
    const Node20 = new TreeNode(20, Node15, Node7);
    const Node9 = new TreeNode(9, null, null);
    const NodeRoot = new TreeNode(3, Node9, Node20);
    expect(levelOrder(NodeRoot)).toEqual([[3], [9, 20], [15, 7]]);
  });

  test('Case 2', () => {
    const NodeRoot = new TreeNode(1, null, null);
    expect(levelOrder(NodeRoot)).toEqual([[1]]);
  });

  test('Case 3', () => {
    expect(levelOrder(null)).toEqual([]);
  });
});
