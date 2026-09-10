// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left),
 * right(right) {}
 * };
 */

class Solution {
   public:
    pair<int, int> go(TreeNode* root, int& ans) {
        if (!root) {
            return {0, 0};
        }

        auto left = go(root->left, ans);
        auto right = go(root->right, ans);

        auto size = left.second + right.second + 1;
        auto sum = left.first + right.first + root->val;
        ans += (sum / size == root->val);

        return {sum, size};
    }

    int averageOfSubtree(TreeNode* root) {
        int ans = 0;
        go(root, ans);

        return ans;
    }
};