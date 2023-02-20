- Date:
    - Change Reason:
    - Previous Snippet:
    - Current Snippet:


- Date: Feb 19, 2023
    - Change Reason: Current block hash generation does redundant work
    - Previous Snippet:
        - Generated 4 hashes by  
            ```
            let mut hasher = Sha256::new();
            hasher.input(&data[..]);
            ```
        - ```
          0000e1640e884eb09b40af7f9b84ece86d9a37fbb46591491715ecee1a4f74fc
          00008f8dd6df6faedbaeb52a5ce0e2090f76d594435539c35bbad1ac64a40db2
          0000735fa1db2dd6717f497888eb848b195b9dd2383fedab1b696ec580ed983f
          0000ec1779bb3a16031dfbf239ae2b7c9d931765075959c14a515d9ffc44bf35
          ```
    - Current Snippet:
      - Later